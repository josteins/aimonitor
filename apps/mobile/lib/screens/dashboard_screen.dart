import 'package:flutter/material.dart';
import '../models/provider_usage.dart';
import '../widgets/provider_card.dart';

class DashboardScreen extends StatefulWidget {
  const DashboardScreen({super.key});

  @override
  State<DashboardScreen> createState() => _DashboardScreenState();
}

class _DashboardScreenState extends State<DashboardScreen> {
  List<ProviderUsage> providers = [];
  bool isLoading = true;

  @override
  void initState() {
    super.initState();
    _loadProviders();
  }

  Future<void> _loadProviders() async {
    await Future.delayed(const Duration(seconds: 1));

    setState(() {
      providers = [
        ProviderUsage(
          provider: Provider(
            id: 'openai_1',
            name: 'OpenAI Production',
            providerType: 'openai',
            enabled: true,
          ),
          todayTokens: 15420,
          todayCost: 0.85,
          mtdTokens: 450000,
          mtdCost: 24.50,
        ),
        ProviderUsage(
          provider: Provider(
            id: 'anthropic_1',
            name: 'Anthropic Claude',
            providerType: 'anthropic',
            enabled: true,
          ),
          todayTokens: 8200,
          todayCost: 0.42,
          mtdTokens: 280000,
          mtdCost: 15.20,
        ),
      ];
      isLoading = false;
    });
  }

  double get totalTodayCost {
    return providers.fold(0, (sum, p) => sum + p.todayCost);
  }

  double get totalMTDCost {
    return providers.fold(0, (sum, p) => sum + p.mtdCost);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('AI Usage Monitor'),
        actions: [
          IconButton(
            icon: const Icon(Icons.settings),
            onPressed: () {},
          ),
        ],
      ),
      body: isLoading
          ? const Center(child: CircularProgressIndicator())
          : RefreshIndicator(
              onRefresh: _loadProviders,
              child: CustomScrollView(
                slivers: [
                  SliverToBoxAdapter(
                    child: Container(
                      padding: const EdgeInsets.all(16),
                      child: Card(
                        child: Padding(
                          padding: const EdgeInsets.all(16),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                'Total Usage',
                                style: Theme.of(context).textTheme.titleMedium,
                              ),
                              const SizedBox(height: 16),
                              Row(
                                mainAxisAlignment: MainAxisAlignment.spaceAround,
                                children: [
                                  Column(
                                    children: [
                                      Text(
                                        'Today',
                                        style: Theme.of(context).textTheme.bodySmall,
                                      ),
                                      const SizedBox(height: 4),
                                      Text(
                                        '\$${totalTodayCost.toStringAsFixed(2)}',
                                        style: Theme.of(context)
                                            .textTheme
                                            .headlineSmall
                                            ?.copyWith(
                                              fontWeight: FontWeight.bold,
                                            ),
                                      ),
                                    ],
                                  ),
                                  Column(
                                    children: [
                                      Text(
                                        'Month to Date',
                                        style: Theme.of(context).textTheme.bodySmall,
                                      ),
                                      const SizedBox(height: 4),
                                      Text(
                                        '\$${totalMTDCost.toStringAsFixed(2)}',
                                        style: Theme.of(context)
                                            .textTheme
                                            .headlineSmall
                                            ?.copyWith(
                                              fontWeight: FontWeight.bold,
                                            ),
                                      ),
                                    ],
                                  ),
                                ],
                              ),
                            ],
                          ),
                        ),
                      ),
                    ),
                  ),
                  SliverPadding(
                    padding: const EdgeInsets.symmetric(horizontal: 16),
                    sliver: SliverList(
                      delegate: SliverChildBuilderDelegate(
                        (context, index) {
                          return ProviderCard(usage: providers[index]);
                        },
                        childCount: providers.length,
                      ),
                    ),
                  ),
                ],
              ),
            ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {},
        child: const Icon(Icons.add),
      ),
    );
  }
}