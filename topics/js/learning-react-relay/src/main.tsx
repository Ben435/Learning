import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './index.css'
import { RelayEnvironmentProvider } from 'react-relay'
import {
  Environment,
  Network,
  Store,
  RecordSource,
  type RequestParameters,
  type Variables,
  type GraphQLResponse
} from 'relay-runtime'
import rawQueryMap from '../persisted_queries.json'

async function fetchQuery(
  params: RequestParameters,
  variables: Variables,
): Promise<GraphQLResponse> {
  let response = await fetch('https://swapi-graphql.netlify.app/.netlify/functions/index', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      kind: params.operationKind,
      operationName: params.name,
      id: params.id,
      variables,
    }),
  });

  // APQ handshake -> https://www.apollographql.com/docs/apollo-server/performance/apq/
  if (!response.ok) {
    console.warn(`server did not not query with id ${params.id}, sending raw query`)

    const rawQuery = (rawQueryMap as Record<string, string>)[params.id!]
    response = await fetch('https://swapi-graphql.netlify.app/.netlify/functions/index', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        kind: params.operationKind,
        operationName: params.name,
        id: params.id,
        query: rawQuery,
        variables,
      }),
    })
  }

  return response.json();
}

const env: Environment = new Environment({
  network: Network.create(fetchQuery),
  store: new Store(new RecordSource()),
});

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <RelayEnvironmentProvider environment={env}>
      <App />
    </RelayEnvironmentProvider>
  </React.StrictMode>,
)
