import React from 'react';
import { Sparkline } from './Sparkline';
import '../styles/ProviderCard.css';

interface ProviderUsage {
  provider: {
    id: string;
    name: string;
    provider_type: string;
    enabled: boolean;
  };
  today_tokens: number;
  today_cost: number;
  mtd_tokens: number;
  mtd_cost: number;
  balance?: number;
  credits?: number;
  budget_used_percentage?: number;
}

interface ProviderCardProps {
  usage: ProviderUsage;
  onToggle: (enabled: boolean) => void;
  onRemove: () => void;
}

export const ProviderCard: React.FC<ProviderCardProps> = ({ usage, onToggle, onRemove }) => {
  const getProviderIcon = () => {
    switch (usage.provider.provider_type) {
      case 'openai':
        return '🤖';
      case 'anthropic':
        return '🧠';
      case 'openrouter':
        return '🌐';
      default:
        return '📊';
    }
  };

  const getStatusColor = () => {
    if (!usage.provider.enabled) return 'disabled';
    if (!usage.budget_used_percentage) return 'healthy';
    if (usage.budget_used_percentage >= 90) return 'critical';
    if (usage.budget_used_percentage >= 75) return 'warning';
    return 'healthy';
  };

  const formatTokens = (tokens: number) => {
    if (tokens >= 1000000) return `${(tokens / 1000000).toFixed(1)}M`;
    if (tokens >= 1000) return `${(tokens / 1000).toFixed(1)}K`;
    return tokens.toString();
  };

  return (
    <div className={`provider-card ${getStatusColor()}`}>
      <div className="provider-header">
        <div className="provider-title">
          <span className="provider-icon">{getProviderIcon()}</span>
          <h3>{usage.provider.name}</h3>
        </div>
        <div className="provider-actions">
          <label className="toggle-switch">
            <input
              type="checkbox"
              checked={usage.provider.enabled}
              onChange={(e) => onToggle(e.target.checked)}
            />
            <span className="toggle-slider"></span>
          </label>
          <button className="remove-btn" onClick={onRemove} title="Remove provider">
            ×
          </button>
        </div>
      </div>

      <div className="provider-stats">
        <div className="stat-row">
          <div className="stat">
            <span className="label">Today</span>
            <span className="value">${usage.today_cost.toFixed(2)}</span>
            <span className="sub-label">{formatTokens(usage.today_tokens)} tokens</span>
          </div>
          <div className="stat">
            <span className="label">MTD</span>
            <span className="value">${usage.mtd_cost.toFixed(2)}</span>
            <span className="sub-label">{formatTokens(usage.mtd_tokens)} tokens</span>
          </div>
        </div>

        {(usage.balance !== undefined || usage.credits !== undefined) && (
          <div className="balance-row">
            {usage.credits !== undefined && (
              <div className="stat">
                <span className="label">Credits</span>
                <span className="value">{usage.credits.toFixed(2)}</span>
              </div>
            )}
            {usage.balance !== undefined && (
              <div className="stat">
                <span className="label">Balance</span>
                <span className="value">${usage.balance.toFixed(2)}</span>
              </div>
            )}
          </div>
        )}

        {usage.budget_used_percentage !== undefined && (
          <div className="budget-progress">
            <div className="budget-header">
              <span className="label">Budget Used</span>
              <span className="percentage">{usage.budget_used_percentage.toFixed(0)}%</span>
            </div>
            <div className="progress-bar">
              <div
                className="progress-fill"
                style={{ width: `${Math.min(usage.budget_used_percentage, 100)}%` }}
              />
            </div>
          </div>
        )}
      </div>

      <div className="sparkline-container">
        <Sparkline data={[]} label="24h Usage" />
      </div>
    </div>
  );
};