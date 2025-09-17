import { Env } from '../index';
import { pollOpenAI } from '../providers/openai';
import { pollAnthropic } from '../providers/anthropic';
import { pollOpenRouter } from '../providers/openrouter';

export async function handleProviderPoll(request: Request, env: Env) {
  const provider = (request as any).params.provider;
  const body = await request.json() as { apiKey: string };

  if (!body.apiKey) {
    return new Response('API key required', { status: 400 });
  }

  try {
    let usage;

    switch (provider) {
      case 'openai':
        usage = await pollOpenAI(body.apiKey);
        break;
      case 'anthropic':
        usage = await pollAnthropic(body.apiKey);
        break;
      case 'openrouter':
        usage = await pollOpenRouter(body.apiKey);
        break;
      default:
        return new Response(`Unknown provider: ${provider}`, { status: 400 });
    }

    return new Response(JSON.stringify(usage), {
      headers: { 'Content-Type': 'application/json' },
    });
  } catch (error: any) {
    return new Response(JSON.stringify({ error: error.message }), {
      status: 500,
      headers: { 'Content-Type': 'application/json' },
    });
  }
}