import { useQueryLoader } from 'react-relay'
import './App.css'
import { Movies } from './movies/Movies'
import { People } from './people/People'
import { Suspense, useEffect } from 'react'
import { appMainQuery } from './__generated__/appMainQuery.graphql'
import { appQuery } from './app'

function App() {
  const [queryReference, loadQuery, disposeQuery] = useQueryLoader<appMainQuery>(appQuery, null)

  useEffect(() => {
    if (queryReference == null) {
      loadQuery({})
    }
  }, [queryReference, loadQuery, disposeQuery])

  return (
    <>
      <h1>Star Wars stuff</h1>
      <div className="container">
        <div className="card">
          <Suspense fallback="loading...">
            {queryReference ? <Movies queryReference={queryReference} /> : null}
          </Suspense>

        </div>
        <div className="card">
          <Suspense fallback="loading...">
            {queryReference ? <People queryReference={queryReference} /> : null}
          </Suspense>
        </div>
      </div>
    </>
  )
}

export default App
