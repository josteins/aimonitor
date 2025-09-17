import React, { useState } from 'react';
import '../styles/Modal.css';

interface AddProviderModalProps {
  onAdd: (provider: {
    provider_type: string;
    api_key: string;
    name: string;
  }) => void;
  onClose: () => void;
}

export const AddProviderModal: React.FC<AddProviderModalProps> = ({ onAdd, onClose }) => {
  const [providerType, setProviderType] = useState('openai');
  const [apiKey, setApiKey] = useState('');
  const [name, setName] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (apiKey && name) {
      onAdd({
        provider_type: providerType,
        api_key: apiKey,
        name: name,
      });
    }
  };

  const getProviderPlaceholder = () => {
    switch (providerType) {
      case 'openai':
        return 'sk-...';
      case 'anthropic':
        return 'sk-ant-...';
      case 'openrouter':
        return 'sk-or-...';
      default:
        return 'API Key';
    }
  };

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2>Add AI Provider</h2>
          <button className="modal-close" onClick={onClose}>×</button>
        </div>

        <form onSubmit={handleSubmit}>
          <div className="form-group">
            <label htmlFor="provider-type">Provider Type</label>
            <select
              id="provider-type"
              value={providerType}
              onChange={(e) => setProviderType(e.target.value)}
            >
              <option value="openai">OpenAI</option>
              <option value="anthropic">Anthropic</option>
              <option value="openrouter">OpenRouter</option>
            </select>
          </div>

          <div className="form-group">
            <label htmlFor="name">Display Name</label>
            <input
              id="name"
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g., Production API"
              required
            />
          </div>

          <div className="form-group">
            <label htmlFor="api-key">API Key</label>
            <input
              id="api-key"
              type="password"
              value={apiKey}
              onChange={(e) => setApiKey(e.target.value)}
              placeholder={getProviderPlaceholder()}
              required
            />
            <small className="help-text">
              Your API key will be securely stored in your system's keychain
            </small>
          </div>

          <div className="modal-actions">
            <button type="button" className="btn-secondary" onClick={onClose}>
              Cancel
            </button>
            <button type="submit" className="btn-primary">
              Add Provider
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};