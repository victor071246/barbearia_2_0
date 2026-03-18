import { useEffect, useState } from 'react';
import { useAuthStore } from '../store/authStore';
import { api } from '../api/client';
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

  return (
    <div className={styles.container}>
      <div className={styles.filtros}>
        {['Todos', 'Serviços', 'Produtos'].map((cat) => (
          <button
            key={cat}
            className={`${styles.filtro} ${categoria === cat ? styles.ativo : ''}`}
            onClick={() => setCategoria(cat)}
          >
            {cat.charAt(0).toUpperCase() + cat.slice(1)}
          </button>
        ))}
      </div>

      <div className={styles.grid}>
        {items.map((item) => (
          <div key={item.id} className={styles.card}>
            {item.image_url && (
              <img
                src={item.image_url}
                alt={item.nome}
                className={styles.imagem}
              ></img>
            )}
            <div className={styles.info}>
              <h3>{item.nome}</h3>
              <p>R$ {item.preco.toFixed(2)}</p>
              {item.descricao && <p>{item.descricao}</p>}
            </div>
            {isAuthenticated && (
              <button className={styles.editar}>Editar</button>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
