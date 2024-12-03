import './App.css'

import { invoke } from '@tauri-apps/api/core';
import { createSignal } from 'solid-js';



function App() {

  const [devices, setDevices] = createSignal<string[]>([]);
  const [loading, setLoading] = createSignal(false);

  async function handleSearchDevices() {
    try {
      setLoading(true);
      const resultsDevices: string[] = await invoke('search_devices');
      setDevices(resultsDevices);
    } catch(error){
      console.error('erro ao buscar dispositivos: ', error);
    }finally {
      setLoading(false);
    }
  }


  return (
    <>
      <div>
        <h1>descoberta de dispositivos</h1>
        <button onclick={handleSearchDevices} disabled={loading()}>
          {loading() ? 'procurando...' : 'procurar dispositivos'}
        </button>
        <div>
          <h2>dispositivos encontrados:</h2>
          <ul>
            {devices().map(device => (
              <li>{device}</li>
            ))}
          </ul>
        </div>
      </div>
    </>
  )
}

export default App