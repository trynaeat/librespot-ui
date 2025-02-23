import './scss/styles.scss';
import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { useLibreInfo } from './hooks/librespot';
import { QueryClient } from '@tanstack/react-query';
import { PersistQueryClientProvider } from '@tanstack/react-query-persist-client';
import { createSyncStoragePersister } from '@tanstack/query-sync-storage-persister';
import { LibreStatus } from './components/libreStatus';
import { LibreButtons } from './components/libreButtons';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      gcTime: 1000 * 60 * 60 * 24,
    }
  }
});

const persister = createSyncStoragePersister({
  storage: window.localStorage,
});

function App() {
  return (
    <>
      <PersistQueryClientProvider
        client={queryClient}
        persistOptions={{persister}}>
        <nav className="navbar navbar-expand-lg navbar-light bg-light">
          <div className='container-fluid'>
            <a className='navbar-brand' href='#'>Librespot-UI</a>
          </div>
        </nav>
        <div className='pb-3 pt-5'><LibreStatus></LibreStatus></div>
        <LibreButtons></LibreButtons>
      </PersistQueryClientProvider>
    </>
  )
}

export default App
