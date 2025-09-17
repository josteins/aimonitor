export async function pollAnthropic(apiKey: string) {
  const now = new Date();
  const startOfMonth = new Date(now.getFullYear(), now.getMonth(), 1);

  const usageResponse = await fetch(
    `https://api.anthropic.com/v1/organizations/usage_report/messages?` +
    `start_date=${startOfMonth.toISOString().split('T')[0]}&` +
    `end_date=${now.toISOString().split('T')[0]}`,
    {
      method: 'GET',
      headers: {
        'x-api-key': apiKey,
        'anthropic-version': '2023-06-01',
      },
      cf: {
        cacheTtl: 60,
      },
    }
  );

  if (!usageResponse.ok) {
    throw new Error(`Anthropic API error: ${usageResponse.status}`);
  }

  const usageData = await usageResponse.json();

  const todayUsage = usageData.usage.filter((item: any) => {
    const timestamp = new Date(item.timestamp);
    return timestamp.toDateString() === now.toDateString();
  });

  const todayTokens = todayUsage.reduce(
    (sum: number, item: any) => sum + item.input_tokens + item.output_tokens,
    0
  );

  const mtdTokens = usageData.usage.reduce(
    (sum: number, item: any) => sum + item.input_tokens + item.output_tokens,
    0
  );

  const costResponse = await fetch(
    `https://api.anthropic.com/v1/organizations/cost_report?` +
    `start_date=${startOfMonth.toISOString().split('T')[0]}&` +
    `end_date=${now.toISOString().split('T')[0]}`,
    {
      method: 'GET',
      headers: {
        'x-api-key': apiKey,
        'anthropic-version': '2023-06-01',
      },
      cf: {
        cacheTtl: 60,
      },
    }
  );

  const costData = await costResponse.json();

  const todayCost = costData.costs
    .filter((item: any) => {
      const timestamp = new Date(item.timestamp);
      return timestamp.toDateString() === now.toDateString();
    })
    .reduce((sum: number, item: any) => sum + item.amount, 0);

  const mtdCost = costData.costs.reduce((sum: number, item: any) => sum + item.amount, 0);

  return {
    todayTokens,
    todayCost,
    mtdTokens,
    mtdCost,
    provider: 'anthropic',
  };
}