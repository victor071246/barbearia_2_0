import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useState } from 'react';
import { faXmark } from '@fortawesome/free-solid-svg-icons';
import { api } from '../../api/client';
import { ImageCropper } from './ImageCropper';
import styles from './CreateItemModal.module.css';

interface CreateItemModalProps {
  isOpen: boolean;
  onClose: () => void;
  onCreated: () => void;
}

export function CreateItemModal({
  isOpen,
  onClose,
  onCreated,
}: CreateItemModalProps) {
  const [nome, setNome] = useState('');
  const [descricao, setDescricao] = useState('');
  const [preco, setPreco] = useState('');
  const [tipo, setTipo] = useState('produto');
  const [estoqueAtual, setEstoqueAtual] = useState('');
  const [estoqueMinimo, setEstoqueMinimo] = useState('');
  const [imagem, setImagem] = useState<File | null>(null);
  const [linkPagamento, setLinkPagamento] = useState('');
  const [error, setError] = useState('');
  const [imagemParaCropper, setImagemParaCropper] = useState<string | null>(
    null,
  );
  const [showCropper, setShowCropper] = useState(false);

  const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    const url = URL.createObjectURL(file);
    setImagemParaCropper(url);
    setShowCropper(true);
  };

  const handleCropConfirm = (croppedFile: File) => {
    setImagem(croppedFile);
    setShowCropper(false);
    if (imagemParaCropper) URL.revokeObjectURL(imagemParaCropper);
    setImagemParaCropper(null);
  };

  const handleCropCancel = () => {
    setShowCropper(false);
    if (imagemParaCropper) URL.revokeObjectURL(imagemParaCropper);
    setImagemParaCropper(null);
  };

  const handleSubmit = async () => {
    try {
      const form = new FormData();
      form.append('nome', nome);
      form.append('descricao', descricao);
      form.append('preco', preco);
      form.append('tipo', tipo);
      form.append('estoque_atual', estoqueAtual);
      form.append('estoque_minimo', estoqueMinimo);
      form.append('link_pagamento', linkPagamento);
      if (imagem) form.append('image', imagem);

      await api.post('/items', form, {
        headers: { 'Content-Type': 'multipart/form-data' },
      });

      onCreated();
      onClose();
    } catch {
      setError('Erro ao criar item');
    }
  };

  return (
    <>
      <div className={`${styles.panel} ${isOpen ? styles.open : ''}`}>
        <div className={styles.header}>
          <span className={styles.title}>Novo Item</span>
          <FontAwesomeIcon
            icon={faXmark}
            className={styles.closeIcon}
            onClick={onClose}
          ></FontAwesomeIcon>
        </div>

        <div className={styles.form}>
          <input
            className={styles.input}
            placeholder="Nome"
            value={nome}
            onChange={(e) => setNome(e.target.value)}
          />
          <input
            className={styles.input}
            placeholder="Descrição"
            value={descricao}
            onChange={(e) => setDescricao(e.target.value)}
          />
          <input
            className={styles.input}
            placeholder="Preço"
            type="number"
            value={preco}
            onChange={(e) => setPreco(e.target.value)}
          />

          <select
            className={styles.input}
            value={tipo}
            onChange={(e) => setTipo(e.target.value)}
          >
            <option value="produto">Produtos</option>
            <option value="servico">Serviços</option>
          </select>

          <input
            className={styles.input}
            placeholder="Estoque atual"
            type="number"
            value={estoqueAtual}
            onChange={(e) => setEstoqueAtual(e.target.value)}
          />
          <input
            className={styles.input}
            placeholder="Estoque mínimo"
            type="number"
            value={estoqueMinimo}
            onChange={(e) => setEstoqueMinimo(e.target.value)}
          />
          <input
            className={styles.input}
            placeholder="Link extero"
            value={linkPagamento}
            onChange={(e) => setLinkPagamento(e.target.value)}
          ></input>

          <input
            className={styles.input}
            type="file"
            accept="image/*"
            onChange={handleFileSelect}
          />

          {imagem && (
            <img
              src={URL.createObjectURL(imagem)}
              alt="Preview"
              style={{
                maxWidth: '100%',
                borderRadius: '8px',
                marginTop: '8px',
              }}
            ></img>
          )}

          {error && <span className={styles.error}>{error}</span>}

          <button className={styles.button} onClick={handleSubmit}>
            Criar
          </button>
        </div>
      </div>

      {showCropper && imagemParaCropper && (
        <ImageCropper
          imageSrc={imagemParaCropper}
          onConfirm={handleCropConfirm}
          onCancel={handleCropCancel}
          aspectRatio={4 / 3}
        ></ImageCropper>
      )}
    </>
  );
}
