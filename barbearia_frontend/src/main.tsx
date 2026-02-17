import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { TopBar } from './components/layout/TopBar';
import { NavBar } from './components/layout/NavBar';
import { Home } from './pages/Home';
import { ContactPage } from './pages/ContactPage';
import { ItemsPage } from './pages/ItemsPage';
import './index.css';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <BrowserRouter>
      <TopBar></TopBar>
      <NavBar></NavBar>
      <Routes>
        <Route path="/" element={<Home></Home>}></Route>
        <Route path="/servicos" element={<ItemsPage></ItemsPage>}></Route>
        <Route path="/contato" element={<ContactPage></ContactPage>}></Route>
      </Routes>
    </BrowserRouter>
  </StrictMode>,
);
