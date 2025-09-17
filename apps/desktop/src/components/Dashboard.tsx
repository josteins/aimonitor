import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { ProviderCard } from './ProviderCard';
import { AddProviderModal } from './AddProviderModal';
import '../styles/Dashboard.css';

interface Provider {
  id: string;
  name: string;
  provider_type: 'openai' | 'anthropic' | 'openrouter';
  enabled: boolean;
}

interface ProviderUsage {
  provider: Provider;
  today_tokens: number;
  today_cost: number;
  mtd_tokens: number;
  mtd_cost: number;
  balance?: number;
  credits?: number;
  budget_used_percentage?: number;
}

export const Dashboard: React.FC = () => {
  const [providers, setProviders] = useState<ProviderUsage[]>([]);
  const [loading, setLoading] = useState(true);
  const [showAddProvider, setShowAddProvider] = useState(false);
  const [refreshInterval, setRefreshInterval] = useState<NodeJS.Timeout | null>(null);

  const fetchUsage = async () => {
    try {
      const response = await invoke<{ providers: ProviderUsage[] }>('get_usage');
      setProviders(response.providers);
      setLoading(false);
    } catch (error) {
      console.error('Failed to fetch usage:', error);
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchUsage();

    const interval = setInterval(fetchUsage, 60000);
    setRefreshInterval(interval);

    return () => {
      if (refreshInterval) clearInterval(refreshInterval);
    };
  }, []);

  const handleAddProvider = async (providerData: {
    provider_type: string;
    api_key: string;
    name: string;
  }) => {
    try {
      await invoke('add_provider', { request: providerData });
      await fetchUsage();
      setShowAddProvider(false);
    } catch (error) {
      console.error('Failed to add provider:', error);
    }
  };

  const handleRemoveProvider = async (providerId: string) => {
    try {
      await invoke('remove_provider', { providerId });
      await fetchUsage();
    } catch (error) {
      console.error('Failed to remove provider:', error);
    }
  };

  const handleToggleProvider = async (providerId: string, enabled: boolean) => {
    try {
      await invoke('toggle_provider', { providerId, enabled });
      await fetchUsage();
    } catch (error) {
      console.error('Failed to toggle provider:', error);
    }
  };

  const getTotalMTDCost = () => {
    return providers.reduce((sum, p) => sum + p.mtd_cost, 0);
  };

  const getTotalTodayCost = () => {
    return providers.reduce((sum, p) => sum + p.today_cost, 0);
  };

  if (loading) {
    return (
      <div className="dashboard-loading">
        <div className="spinner" />
        <p>Loading usage data...</p>
      </div>
    );
  }

  return (
    <div className="dashboard">
      <header className="dashboard-header">
        <h1>AI Usage Monitor</h1>
        <div className="header-stats">
          <div className="stat">
            <span className="stat-label">Today</span>
            <span className="stat-value">${getTotalTodayCost().toFixed(2)}</span>
          </div>
          <div className="stat">
            <span className="stat-label">Month to Date</span>
            <span className="stat-value">${getTotalMTDCost().toFixed(2)}</span>
          </div>
        </div>
        <button
          className="add-provider-btn"
          onClick={() => setShowAddProvider(true)}
        >
          + Add Provider
        </button>
      </header>

      <div className="providers-grid">
        {providers.map((usage) => (
          <ProviderCard
            key={usage.provider.id}
            usage={usage}
            onToggle={(enabled) => handleToggleProvider(usage.provider.id, enabled)}
            onRemove={() => handleRemoveProvider(usage.provider.id)}
          />
        ))}

        {providers.length === 0 && (
          <div className="no-providers">
            <p>No providers configured</p>
            <button onClick={() => setShowAddProvider(true)}>
              Add your first provider
            </button>
          </div>
        )}
      </div>

      {showAddProvider && (
        <AddProviderModal
          onAdd={handleAddProvider}
          onClose={() => setShowAddProvider(false)}
        />
      )}
    </div>
  );
};