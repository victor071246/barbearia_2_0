import { createRoot } from 'react-dom/client';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { TopBar } from './components/layout/TopBar';
import { NavBar } from './components/layout/NavBar';
import { Home } from './pages/Home';
import { ItemsPage } from './pages/ItemsPage';
import { ContactPage } from './pages/ContactPage';
import { StrictMode } from 'react';
import './index.css';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <BrowserRouter>
      <TopBar />
      <NavBar />
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/servicos" element={<ItemsPage />} />
        <Route path="/contato" element={<ContactPage />} />
      </Routes>
    </BrowserRouter>
  </StrictMode>,
);
