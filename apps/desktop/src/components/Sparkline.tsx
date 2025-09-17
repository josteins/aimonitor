import React from 'react';

interface SparklineProps {
  data: number[];
  label: string;
  width?: number;
  height?: number;
}

export const Sparkline: React.FC<SparklineProps> = ({
  data,
  label,
  width = 200,
  height = 40
}) => {
  const generateMockData = () => {
    return Array.from({ length: 24 }, () => Math.random() * 100);
  };

  const values = data.length > 0 ? data : generateMockData();
  const max = Math.max(...values);
  const min = Math.min(...values);
  const range = max - min || 1;

  const points = values.map((value, index) => {
    const x = (index / (values.length - 1)) * width;
    const y = height - ((value - min) / range) * height;
    return `${x},${y}`;
  }).join(' ');

  return (
    <div className="sparkline">
      <span className="sparkline-label">{label}</span>
      <svg width={width} height={height} className="sparkline-svg">
        <polyline
          points={points}
          fill="none"
          stroke="var(--accent-color)"
          strokeWidth="2"
        />
        <polyline
          points={`0,${height} ${points} ${width},${height}`}
          fill="url(#gradient)"
          fillOpacity="0.3"
          stroke="none"
        />
        <defs>
          <linearGradient id="gradient" x1="0%" y1="0%" x2="0%" y2="100%">
            <stop offset="0%" stopColor="var(--accent-color)" />
            <stop offset="100%" stopColor="var(--accent-color)" stopOpacity="0" />
          </linearGradient>
        </defs>
      </svg>
    </div>
  );
};