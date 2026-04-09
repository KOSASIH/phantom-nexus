import React, { Suspense } from 'react';
import { Canvas } from '@react-three/fiber';
import { OrbitControls, Stars, Text } from '@react-three/drei';

function PhantomNexusScene() {
  return (
    <>
      <ambientLight intensity={0.3} />
      <pointLight position={[10, 10, 10]} intensity={1} color="#00d4ff" />
      <pointLight position={[-10, -10, -10]} intensity={0.5} color="#7b2fff" />

      <Stars radius={300} depth={50} count={5000} factor={4} saturation={0} fade speed={1} />

      {/* Central nexus sphere */}
      <mesh position={[0, 0, 0]}>
        <sphereGeometry args={[2, 64, 64]} />
        <meshStandardMaterial
          color="#0a0a2e"
          emissive="#00d4ff"
          emissiveIntensity={0.3}
          wireframe
          transparent
          opacity={0.8}
        />
      </mesh>

      {/* Inner core */}
      <mesh position={[0, 0, 0]}>
        <icosahedronGeometry args={[1.2, 2]} />
        <meshStandardMaterial
          color="#7b2fff"
          emissive="#7b2fff"
          emissiveIntensity={0.5}
          wireframe
        />
      </mesh>

      {/* Orbiting nodes */}
      {Array.from({ length: 12 }).map((_, i) => {
        const angle = (i / 12) * Math.PI * 2;
        const radius = 4;
        return (
          <mesh
            key={i}
            position={[
              Math.cos(angle) * radius,
              Math.sin(angle * 0.5) * 1.5,
              Math.sin(angle) * radius,
            ]}
          >
            <octahedronGeometry args={[0.2]} />
            <meshStandardMaterial
              color="#00d4ff"
              emissive="#00d4ff"
              emissiveIntensity={0.8}
            />
          </mesh>
        );
      })}

      <OrbitControls enableZoom autoRotate autoRotateSpeed={0.5} />
    </>
  );
}

function Dashboard() {
  return (
    <div style={{
      position: 'absolute',
      top: 0,
      left: 0,
      width: '100%',
      height: '100%',
      pointerEvents: 'none',
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'space-between',
      padding: '2rem',
      zIndex: 10,
    }}>
      {/* Header */}
      <div>
        <h1 style={{ fontSize: '2.5rem', fontWeight: 700, margin: 0, color: '#00d4ff' }}>
          👻 Phantom Nexus
        </h1>
        <p style={{ color: '#888', margin: '0.5rem 0', fontSize: '1.1rem' }}>
          Invisible Intelligence, Infinite Scale
        </p>
      </div>

      {/* Stats */}
      <div style={{
        display: 'flex',
        gap: '2rem',
        pointerEvents: 'auto',
      }}>
        {[
          { label: 'TPS', value: '1,247,832', color: '#00d4ff' },
          { label: 'Active Nodes', value: '12,847', color: '#7b2fff' },
          { label: 'Total Staked', value: '$4.2B', color: '#00ff88' },
          { label: 'AI Agents', value: '8,192', color: '#ff6b00' },
          { label: '$PHNX Price', value: '$12.47', color: '#00d4ff' },
        ].map((stat) => (
          <div key={stat.label} style={{
            background: 'rgba(10, 10, 30, 0.8)',
            border: `1px solid ${stat.color}33`,
            borderRadius: '12px',
            padding: '1rem 1.5rem',
            backdropFilter: 'blur(10px)',
          }}>
            <div style={{ color: '#666', fontSize: '0.85rem' }}>{stat.label}</div>
            <div style={{ color: stat.color, fontSize: '1.5rem', fontWeight: 700 }}>
              {stat.value}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

export default function App() {
  return (
    <div style={{ width: '100%', height: '100vh', background: '#0a0a0f' }}>
      <Canvas camera={{ position: [0, 2, 8], fov: 60 }}>
        <Suspense fallback={null}>
          <PhantomNexusScene />
        </Suspense>
      </Canvas>
      <Dashboard />
    </div>
  );
}
