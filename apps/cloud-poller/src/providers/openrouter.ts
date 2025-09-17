export async function pollOpenRouter(apiKey: string) {
  const keyResponse = await fetch('https://openrouter.ai/api/v1/key', {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${apiKey}`,
    },
    cf: {
      cacheTtl: 60,
    },
  });

  if (!keyResponse.ok) {
    throw new Error(`OpenRouter API error: ${keyResponse.status}`);
  }

  const keyData = await keyResponse.json();

  const creditsResponse = await fetch('https://openrouter.ai/api/v1/credits', {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${apiKey}`,
    },
    cf: {
      cacheTtl: 60,
    },
  });

  const creditsData = await creditsResponse.json();

  return {
    todayTokens: 0,
    todayCost: 0,
    mtdTokens: 0,
    mtdCost: creditsData.used_credits || 0,
    balance: keyData.data.limit_remaining,
    credits: creditsData.remaining_credits,
    provider: 'openrouter',
  };
}