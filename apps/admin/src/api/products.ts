import {
  useMutation,
  useQuery,
  useQueryClient,
  type UseMutationResult,
  type UseQueryResult,
} from "@tanstack/react-query";
import { graphqlRequest } from "./graphql-client";


export interface Product {
  id: string;
  title: string;
  handle: string;
  description: string | null;
  priceCents: number;
  inventoryQuantity: number;
  published: boolean;
  publishedAt: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface CreateProductInput {
  title: string;
  handle: string;
  description?: string;
  priceCents: number;
  inventoryQuantity?: number;
  published?: boolean;
}

const PRODUCTS_QUERY = /* GraphQL */ `
  query Products {
    products {
      id
      title
      handle
      description
      priceCents
      inventoryQuantity
      published
      publishedAt
      createdAt
      updatedAt
    }
  }
`;

const CREATE_PRODUCT_MUTATION = /* GraphQL */ `
  mutation CreateProduct($input:  ProductCreateInput!) {
   productCreate(input: $input) {
      id
      title
      handle
      description
      priceCents
      inventoryQuantity
      published
      publishedAt
      createdAt
      updatedAt
    }
  }
`;

export const productKeys = {
  all: ["products"] as const,
  list: () => [...productKeys.all, "list"] as const,
};

/**
 * Server-state ownership lives here, not in the transport wrapper.
 * TanStack Query gives us loading / error / success states, caching,
 * and (via invalidation below) refetch-on-write without a page reload.
 */
export function useProductsQuery(): UseQueryResult<Product[], Error> {
  return useQuery({
    queryKey: productKeys.list(),
    queryFn: () =>
      graphqlRequest<{ products: Product[] }>(PRODUCTS_QUERY).then(
        (data) => data.products
      ),
  });
}

export function useCreateProductMutation(): UseMutationResult<
  Product,
  Error,
  CreateProductInput
> {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (input: CreateProductInput) =>
      graphqlRequest<
        { createProduct: Product },
        { input: CreateProductInput }
        >
        (CREATE_PRODUCT_MUTATION, { input }).then((data) => data.createProduct),
    onSuccess: () => {
      // Invalidate rather than manually patch the cache: keeps this
      // exercise's cache-ownership story simple and obviously correct.
      queryClient.invalidateQueries({ queryKey: productKeys.list() });
    },
  });
}
