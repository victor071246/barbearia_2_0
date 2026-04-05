import { useState } from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faPencil, faXmark, faCheck } from '@fortawesome/free-solid-svg-icons';
import { ImageCropper } from '../popups/ImageCropper';
import { api } from '../../api/client';
import styles from './ItemCard.module.css';
import { LabeledInput } from '../ui/LabeledInput';
import { LabeledSelect } from '../ui/LabeledSelect';

interface Item {
  id: number;
  nome: string;
  descricao: string | null;
  preco: number;
  image_url: string | null;
  tipo: string | null;
  estoque_atual: number;
  estoque_minimo: number;
  link_pagamento: string | null;
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
  const [tipo, setTipo] = useState(item.tipo ?? 'produto');
  const [linkPagamento, setLinkPagamento] = useState(item.link_pagamento ?? '');

  const [cropperOpen, setCropperOpen] = useState(false);
  const [rawImageSrc, setRawImageSrc] = useState<string | null>(null);
  const [previewUrl, setPreviewUrl] = useState<string | null>(null);

  const handleSave = async () => {
    try {
      const form = new FormData();
      form.append('nome', nome);
      form.append('descricao', descricao);
      form.append('preco', preco);
      form.append('estoque_atual', estoqueAtual);
      form.append('estoque_minimo', estoqueMinimo);
      form.append('tipo', tipo);
      form.append('link_pagamento', linkPagamento);
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
    setTipo(item.tipo ?? 'produto');
    setLinkPagamento(item.link_pagamento ?? '');
  };

  const openLink = (url: string) => {
    if (!url) return;
    const normalizedUrl = url.startsWith('http') ? url : `https://${url}`;
    window.open(normalizedUrl, '_blank');
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
        <div
          className={styles.imageEdit}
          onClick={(e) =>
            (
              e.currentTarget.querySelector(
                'input[type="file"]',
              ) as HTMLInputElement
            )?.click()
          }
        >
          {(previewUrl || item.image_url) && (
            <img
              src={previewUrl ?? item.image_url!}
              alt=""
              className={styles.imagePreview}
            ></img>
          )}
          <span className={styles.imageEditLabel}>Trocar foto</span>
          <input
            id={'fileInput-' + item.id}
            type="file"
            accept="image/*"
            className={styles.fileInput}
            onChange={(e) => {
              const file = e.target.files?.[0];
              if (!file) return;
              const reader = new FileReader();
              reader.onload = () => {
                setRawImageSrc(reader.result as string);
                setCropperOpen(true);
              };
              reader.readAsDataURL(file);
              e.target.value = '';
            }}
          />
        </div>
      ) : (
        item.image_url && (
          <img
            src={item.image_url}
            alt={item.nome}
            className={`${isAuthenticated ? styles.imagem_edicao : styles.imagem}`}
          ></img>
        )
      )}

      <div className={styles.info}>
        {editing ? (
          <>
            <LabeledInput
              label="Nome"
              value={nome}
              onChange={(e) => setNome(e.target.value)}
            ></LabeledInput>
            <LabeledInput
              label="Preço"
              type="number"
              value={preco}
              onChange={(e) => setPreco(e.target.value)}
            ></LabeledInput>
            <LabeledInput
              label="Descrição"
              value={descricao}
              onChange={(e) => setDescricao(e.target.value)}
            ></LabeledInput>
            <LabeledInput
              label="Estoque"
              type="number"
              value={estoqueAtual}
              onChange={(e) => setEstoqueAtual(e.target.value)}
            ></LabeledInput>
            <LabeledInput
              label="Est. mín"
              type="number"
              value={estoqueMinimo}
              onChange={(e) => setEstoqueMinimo(e.target.value)}
            ></LabeledInput>
            <LabeledSelect
              label="Tipo"
              value={tipo}
              onChange={(e) => setTipo(e.target.value)}
            >
              <option value="produto">Produto</option>
              <option value="servico">Serviço</option>
            </LabeledSelect>
            <LabeledInput
              label="Link"
              placeholder={linkPagamento}
              onChange={(e) => setLinkPagamento(e.target.value)}
            ></LabeledInput>
          </>
        ) : (
          <>
            <h3>{item.nome}</h3>
            <p>R$ {Number(item.preco).toFixed(2)}</p>
            {item.descricao && <p>{item.descricao}</p>}
            {item.tipo === 'prouto' && <p>Estoque: {item.estoque_atual}</p>}
            {item.tipo === 'servico' ? (
              <button
                className={styles.btnAgendar}
                onClick={() => openLink(item.link_pagamento!)}
              >
                Agendar
              </button>
            ) : (
              <button
                className={styles.btnComprar}
                onClick={() =>
                  item.link_pagamento && window.open(item.link_pagamento)
                }
              >
                Comprar
              </button>
            )}
          </>
        )}
      </div>
      {cropperOpen && rawImageSrc && (
        <ImageCropper
          imageSrc={rawImageSrc}
          onConfirm={(croppedFile) => {
            setImagem(croppedFile);
            setPreviewUrl(URL.createObjectURL(croppedFile));
            setCropperOpen(false);
            setRawImageSrc(null);
          }}
          onCancel={() => {
            setCropperOpen(false);
            setRawImageSrc(null);
          }}
        ></ImageCropper>
      )}
    </div>
  );
}
