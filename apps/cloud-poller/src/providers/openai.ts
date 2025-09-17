export async function pollOpenAI(apiKey: string) {
  const now = new Date();
  const startOfMonth = new Date(now.getFullYear(), now.getMonth(), 1);

  const response = await fetch('https://api.openai.com/v1/usage', {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${apiKey}`,
      'Content-Type': 'application/json',
    },
    cf: {
      cacheTtl: 60,
    },
  });

  if (!response.ok) {
    throw new Error(`OpenAI API error: ${response.status}`);
  }

  const data = await response.json();

  const todayTokens = data.data
    .filter((item: any) => {
      const timestamp = new Date(item.aggregation_timestamp * 1000);
      return timestamp.toDateString() === now.toDateString();
    })
    .reduce((sum: number, item: any) => {
      return sum + item.n_context_tokens_total + item.n_generated_tokens_total;
    }, 0);

  const mtdTokens = data.data.reduce((sum: number, item: any) => {
    return sum + item.n_context_tokens_total + item.n_generated_tokens_total;
  }, 0);

  const todayCost = data.daily_costs?.[0]?.line_items?.reduce(
    (sum: number, item: any) => sum + item.cost,
    0
  ) || 0;

  const mtdCost = data.daily_costs?.reduce(
    (sum: number, day: any) =>
      sum + day.line_items.reduce((daySum: number, item: any) => daySum + item.cost, 0),
    0
  ) || 0;

  return {
    todayTokens,
    todayCost,
    mtdTokens,
    mtdCost,
    provider: 'openai',
  };
}