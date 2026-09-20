import { useState, type FormEvent } from "react";
import { useCreateProductMutation } from "../api/products";

const emptyForm = {
  title: "",
  handle: "",
  description: "",
  priceCents: "",
  inventoryQuantity: "",
  published: false,
};

export function ProductForm() {
  const [form, setForm] = useState(emptyForm);
  const createProduct = useCreateProductMutation();

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    createProduct.mutate(
      {
        title: form.title,
        handle: form.handle,
        description: form.description || undefined,
        priceCents: Number(form.priceCents),
        inventoryQuantity: form.inventoryQuantity
          ? Number(form.inventoryQuantity)
          : 0,
        published: form.published,
      },
      {
        onSuccess: () => setForm(emptyForm),
      }
    );
  }

  return (
    <form onSubmit={handleSubmit} data-testid="product-create-form">
      <div>
        <label htmlFor="title">Title</label>
        <input
          id="title"
          required
          value={form.title}
          onChange={(e) => setForm({ ...form, title: e.target.value })}
        />
      </div>

      <div>
        <label htmlFor="handle">Handle</label>
        <input
          id="handle"
          required
          value={form.handle}
          onChange={(e) => setForm({ ...form, handle: e.target.value })}
        />
      </div>

      <div>
        <label htmlFor="description">Description</label>
        <textarea
          id="description"
          value={form.description}
          onChange={(e) => setForm({ ...form, description: e.target.value })}
        />
      </div>

      <div>
        <label htmlFor="priceCents">Price (cents)</label>
        <input
          id="priceCents"
          type="number"
          required
          min={0}
          value={form.priceCents}
          onChange={(e) => setForm({ ...form, priceCents: e.target.value })}
        />
      </div>

      <div>
        <label htmlFor="inventoryQuantity">Inventory quantity</label>
        <input
          id="inventoryQuantity"
          type="number"
          min={0}
          value={form.inventoryQuantity}
          onChange={(e) =>
            setForm({ ...form, inventoryQuantity: e.target.value })
          }
        />
      </div>

      <div>
        <label htmlFor="published">
          <input
            id="published"
            type="checkbox"
            checked={form.published}
            onChange={(e) => setForm({ ...form, published: e.target.checked })}
          />
          Published
        </label>
      </div>

      <button type="submit" disabled={createProduct.isPending}>
        {createProduct.isPending ? "Creating…" : "Create product"}
      </button>

      {createProduct.isError && (
        <p role="alert" data-testid="create-error">
          Could not create product: {createProduct.error.message}
        </p>
      )}
      {createProduct.isSuccess && (
        <p data-testid="create-success">Product created.</p>
      )}
    </form>
  );
}
