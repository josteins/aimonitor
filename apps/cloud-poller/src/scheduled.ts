import { Env } from './index';
import { pollOpenAI } from './providers/openai';
import { pollAnthropic } from './providers/anthropic';
import { pollOpenRouter } from './providers/openrouter';
import { sendPushNotification } from './services/push';

interface ProviderConfig {
  userId: string;
  providerId: string;
  providerType: 'openai' | 'anthropic' | 'openrouter';
  apiKey: string;
  softLimit?: number;
  hardLimit?: number;
}

export async function scheduled(env: Env) {
  console.log('Starting scheduled polling');

  const configsJson = await env.KV.get('provider_configs', 'json');
  if (!configsJson) {
    console.log('No provider configs found');
    return;
  }

  const configs = configsJson as ProviderConfig[];

  for (const config of configs) {
    try {
      await pollProvider(env, config);
    } catch (error) {
      console.error(`Failed to poll provider ${config.providerId}:`, error);
    }
  }

  console.log('Scheduled polling complete');
}

async function pollProvider(env: Env, config: ProviderConfig) {
  let usage;

  switch (config.providerType) {
    case 'openai':
      usage = await pollOpenAI(config.apiKey);
      break;
    case 'anthropic':
      usage = await pollAnthropic(config.apiKey);
      break;
    case 'openrouter':
      usage = await pollOpenRouter(config.apiKey);
      break;
    default:
      throw new Error(`Unknown provider type: ${config.providerType}`);
  }

  const previousUsage = await env.KV.get(`usage:${config.userId}:${config.providerId}`, 'json') as any;

  await env.KV.put(
    `usage:${config.userId}:${config.providerId}`,
    JSON.stringify({
      ...usage,
      lastUpdated: new Date().toISOString(),
    })
  );

  await checkAlerts(env, config, usage, previousUsage);
}

async function checkAlerts(env: Env, config: ProviderConfig, currentUsage: any, previousUsage: any) {
  const { softLimit, hardLimit } = config;

  if (!softLimit && !hardLimit) return;

  const mtdCost = currentUsage.mtdCost || 0;
  const shouldAlertSoft = softLimit && mtdCost >= softLimit;
  const shouldAlertHard = hardLimit && mtdCost >= hardLimit;

  if (shouldAlertHard && (!previousUsage || previousUsage.mtdCost < hardLimit)) {
    await sendPushNotification(env, {
      userId: config.userId,
      title: 'Critical: Budget Exceeded',
      body: `${config.providerType} has exceeded hard limit: $${mtdCost.toFixed(2)} / $${hardLimit}`,
      priority: 'high',
    });
  } else if (shouldAlertSoft && (!previousUsage || previousUsage.mtdCost < softLimit)) {
    await sendPushNotification(env, {
      userId: config.userId,
      title: 'Warning: Budget Alert',
      body: `${config.providerType} approaching limit: $${mtdCost.toFixed(2)} / $${softLimit}`,
      priority: 'normal',
    });
  }
}