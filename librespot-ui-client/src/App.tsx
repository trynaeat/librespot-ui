import './scss/styles.scss';
import './App.css'
import { QueryClient } from '@tanstack/react-query';
import { PersistQueryClientProvider } from '@tanstack/react-query-persist-client';
import { createSyncStoragePersister } from '@tanstack/query-sync-storage-persister';
import { LibreStatus } from './components/libreStatus';
import { LibreButtons } from './components/libreButtons';
import { NavBar } from './components/navBar';
import { useUser } from './hooks/user';
import { Home } from './components/home';

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
        <Home></Home>
      </PersistQueryClientProvider>
    </>
  )
}

export default App
