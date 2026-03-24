import { useState } from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faPencil, faXmark, faCheck } from '@fortawesome/free-solid-svg-icons';
import { api } from '../../api/client';
import styles from './ItemCard.module.css';

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

interface ItemCardProps {
  item: Item;
  isAuthenticated: boolean;
  onUpdated: () => void;
}

export function ItemCard({ item, isAuthenticated, onUpdated }: ItemCardProps) {
  const [editing, setEditing] = useState(false);
  const [nome, setNome] = useState(item.nome);
  const [descricao, setDescricao] = useState(item.descricao ?? '');
  const [preco, setPreco] = useState(String(item.preco));
  const [imagem, setImagem] = useState<File | null>(null);
  const [estoqueAtual, setEstoqueAtual] = useState(String(item.estoque_atual));
  const [estoqueMinimo, setEstoqueMinimo] = useState(
    String(item.estoque_minimo),
  );

  const handleSave = async () => {
    try {
      const form = new FormData();
      form.append('nome', nome);
      form.append('descricao', descricao);
      form.append('preco', preco);
      form.append('estoque_atual', String(item.estoque_atual));
      form.append('estoque_minimo', String(item.estoque_minimo));
      if (item.tipo) form.append('tipo', item.tipo);
      if (imagem) form.append('image', imagem);

      await api.put(`/items/${item.id}`, form, {
        headers: { 'Content-Type': 'multipart/form-data' },
      });

      onUpdated();
      setEditing(false);
    } catch (e) {
      console.error(e);
    }
  };

  const handleCancel = () => {
    setNome(item.nome);
    setDescricao(item.descricao ?? '');
    setPreco(String(item.preco));
    setImagem(null);
    setEditing(false);
  };

  return (
    <div className={`${styles.card} ${editing ? styles.editing : ''}`}>
      {isAuthenticated && !editing && (
        <FontAwesomeIcon
          icon={faPencil}
          className={styles.editIcon}
          onClick={() => setEditing(true)}
        ></FontAwesomeIcon>
      )}
      {isAuthenticated && editing && (
        <div className={styles.editActions}>
          <FontAwesomeIcon
            icon={faCheck}
            className={styles.saveIcon}
            onClick={handleSave}
          ></FontAwesomeIcon>
          <FontAwesomeIcon
            icon={faXmark}
            className={styles.cancelIcon}
            onClick={handleCancel}
          ></FontAwesomeIcon>
        </div>
      )}
      {editing ? (
        <div className={styles.imageEdit}>
          <input
            type="file"
            accept="image/*"
            className={styles.fileInput}
            onChange={(e) => setImagem(e.target.files?.[0] ?? null)}
          ></input>
        </div>
      ) : (
        item.image_url && (
          <img
            src={item.image_url}
            alt={item.nome}
            className={styles.imagem}
          ></img>
        )
      )}

      <div className={styles.info}>
        {editing ? (
          <>
            <input
              className={styles.input}
              value={nome}
              onChange={(e) => setNome(e.target.value)}
            ></input>
            <input
              className={styles.input}
              type="number"
              value={preco}
              onChange={(e) => setPreco(e.target.value)}
            ></input>
            <input
              className={styles.input}
              value={descricao}
              onChange={(e) => setDescricao(e.target.value)}
            ></input>
            <input
              className={styles.input}
              type="number"
              value={estoqueAtual}
              onChange={(e) => setEstoqueAtual(e.target.value)}
            ></input>
            <input
              className={styles.input}
              type="number"
              value={estoqueMinimo}
              onChange={(e) => setEstoqueMinimo(e.target.value)}
            ></input>
          </>
        ) : (
          <>
            <h3>{item.nome}</h3>
            <p>R$ {Number(item.preco).toFixed(2)}</p>
            {item.descricao && <p>{item.descricao}</p>}
            <p>Estoque: {item.estoque_atual}</p>
            <p>Mínimo: {item.estoque_minimo}</p>
          </>
        )}
      </div>
    </div>
  );
}
