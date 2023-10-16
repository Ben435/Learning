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

async function fetchQuery(
  params: RequestParameters,
  variables: Variables,
): Promise<GraphQLResponse> {
  const response = await fetch('https://swapi-graphql.netlify.app/.netlify/functions/index', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      operationName: params.name,
      query: params.text,
      variables,
    }),
  });

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
