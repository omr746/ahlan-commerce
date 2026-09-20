
const GRAPHQL_ENDPOINT =
  import.meta.env.VITE_GRAPHQL_ENDPOINT ?? "http://localhost:3000/graphql";

export class GraphQLRequestError extends Error {
  constructor(
    message: string,
    public readonly errors: unknown[]
  ) {
    super(message);
    this.name = "GraphQLRequestError";
  }
}

export async function graphqlRequest<TData, TVariables extends object = {}>(
  query: string,
  variables?: TVariables
): Promise<TData> {
  const response = await fetch(GRAPHQL_ENDPOINT, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ query, variables }),
  });

  if (!response.ok) {
    throw new Error(`GraphQL request failed with status ${response.status}`);
  }

  const json = await response.json();

  if (json.errors) {
    throw new GraphQLRequestError(
      json.errors[0]?.message ?? "GraphQL request returned errors",
      json.errors
    );
  }

  return json.data as TData;
}
