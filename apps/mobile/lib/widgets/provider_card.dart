import 'package:flutter/material.dart';
import '../models/provider_usage.dart';

class ProviderCard extends StatelessWidget {
  final ProviderUsage usage;

  const ProviderCard({super.key, required this.usage});

  String _formatTokens(int tokens) {
    if (tokens >= 1000000) return '${(tokens / 1000000).toStringAsFixed(1)}M';
    if (tokens >= 1000) return '${(tokens / 1000).toStringAsFixed(1)}K';
    return tokens.toString();
  }

  IconData _getProviderIcon() {
    switch (usage.provider.providerType) {
      case 'openai':
        return Icons.smart_toy_outlined;
      case 'anthropic':
        return Icons.psychology_outlined;
      case 'openrouter':
        return Icons.public_outlined;
      default:
        return Icons.analytics_outlined;
    }
  }

  Color _getProviderColor(BuildContext context) {
    switch (usage.provider.providerType) {
      case 'openai':
        return Colors.green;
      case 'anthropic':
        return Colors.orange;
      case 'openrouter':
        return Colors.blue;
      default:
        return Theme.of(context).colorScheme.primary;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.only(bottom: 12),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(
                  _getProviderIcon(),
                  size: 24,
                  color: _getProviderColor(context),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: Text(
                    usage.provider.name,
                    style: Theme.of(context).textTheme.titleMedium?.copyWith(
                          fontWeight: FontWeight.bold,
                        ),
                  ),
                ),
                Switch(
                  value: usage.provider.enabled,
                  onChanged: (value) {},
                ),
              ],
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
                      '\$${usage.todayCost.toStringAsFixed(2)}',
                      style: Theme.of(context).textTheme.titleMedium,
                    ),
                    Text(
                      '${_formatTokens(usage.todayTokens)} tokens',
                      style: Theme.of(context).textTheme.bodySmall?.copyWith(
                            color: Theme.of(context).colorScheme.onSurfaceVariant,
                          ),
                    ),
                  ],
                ),
                Container(
                  width: 1,
                  height: 40,
                  color: Theme.of(context).dividerColor,
                ),
                Column(
                  children: [
                    Text(
                      'MTD',
                      style: Theme.of(context).textTheme.bodySmall,
                    ),
                    const SizedBox(height: 4),
                    Text(
                      '\$${usage.mtdCost.toStringAsFixed(2)}',
                      style: Theme.of(context).textTheme.titleMedium,
                    ),
                    Text(
                      '${_formatTokens(usage.mtdTokens)} tokens',
                      style: Theme.of(context).textTheme.bodySmall?.copyWith(
                            color: Theme.of(context).colorScheme.onSurfaceVariant,
                          ),
                    ),
                  ],
                ),
              ],
            ),
            if (usage.budgetUsedPercentage != null) ...[
              const SizedBox(height: 16),
              Row(
                children: [
                  Text(
                    'Budget Used',
                    style: Theme.of(context).textTheme.bodySmall,
                  ),
                  const Spacer(),
                  Text(
                    '${usage.budgetUsedPercentage!.toStringAsFixed(0)}%',
                    style: Theme.of(context).textTheme.bodySmall,
                  ),
                ],
              ),
              const SizedBox(height: 8),
              LinearProgressIndicator(
                value: usage.budgetUsedPercentage! / 100,
                backgroundColor: Theme.of(context).colorScheme.surfaceVariant,
                valueColor: AlwaysStoppedAnimation<Color>(
                  usage.budgetUsedPercentage! >= 90
                      ? Colors.red
                      : usage.budgetUsedPercentage! >= 75
                          ? Colors.orange
                          : Theme.of(context).colorScheme.primary,
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }
}