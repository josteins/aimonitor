class Provider {
  final String id;
  final String name;
  final String providerType;
  final bool enabled;

  Provider({
    required this.id,
    required this.name,
    required this.providerType,
    required this.enabled,
  });
}

class ProviderUsage {
  final Provider provider;
  final int todayTokens;
  final double todayCost;
  final int mtdTokens;
  final double mtdCost;
  final double? balance;
  final double? credits;
  final double? budgetUsedPercentage;

  ProviderUsage({
    required this.provider,
    required this.todayTokens,
    required this.todayCost,
    required this.mtdTokens,
    required this.mtdCost,
    this.balance,
    this.credits,
    this.budgetUsedPercentage,
  });
}