import { useEffect, useState } from 'react';
import { useAuthStore } from '../store/authStore';
import { api } from '../api/client';
import { CreateItemModal } from '../components/popups/CreateItemModal';
import { ItemCard } from '../components/layout/ItemCard';
import styles from './ItemsPage.module.css';

interface Item {
  id: number;
  nome: string;
  descricao: string | null;
  preco: number;
  image_url: string | null;
  tipo: string | null;
  estoque_atual: number;
  estoque_minimo: number;
}

export function ItemsPage() {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);
  const [items, setItems] = useState<Item[]>([]);
  const [categoria, setCategoria] = useState<string>('todos');
  const [createOpen, setCreateOpen] = useState(false);

  useEffect(() => {
    const fetchItems = async () => {
      try {
        const url =
          categoria === 'todos' ? '/items' : `/items/category/${categoria}`;
        const { data } = await api.get(url);
        setItems(data.data);
      } catch (e) {
        console.error(e);
      }
    };

    fetchItems();
  }, [categoria]);

  const refetch = async () => {
    const url =
      categoria === 'todos' ? '/items' : `/items/category/${categoria}`;
    const { data } = await api.get(url);
    setItems(data.data);
  };

  return (
    <div className={styles.container}>
      <div className={styles.filtros}>
        {['todos', 'servico', 'produto'].map((categoria) => (
          <button
            key={categoria}
            className={`${styles.filtro} ${categoria === categoria ? styles.ativo : ''}`}
            onClick={() => setCategoria(categoria)}
          >
            {categoria.charAt(0).toUpperCase() + categoria.slice(1)}
          </button>
        ))}
      </div>

      {isAuthenticated && (
        <button className={styles.criar} onClick={() => setCreateOpen(true)}>
          +
        </button>
      )}

      <CreateItemModal
        isOpen={createOpen}
        onClose={() => setCreateOpen(false)}
        onCreated={() => {
          setCategoria('todos'); // força refetch
        }}
      />

      <div className={styles.grid}>
        {items.map((item) => (
          <ItemCard
            key={item.id}
            item={item}
            isAuthenticated={isAuthenticated}
            onUpdated={refetch}
          ></ItemCard>
        ))}
      </div>
    </div>
  );
}
