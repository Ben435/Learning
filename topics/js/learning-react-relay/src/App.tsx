import { useQueryLoader } from 'react-relay'
import './App.css'
import { Movies } from './movies/Movies'
import { People } from './people/People'
import { useEffect } from 'react'
import { appMainQuery } from './__generated__/appMainQuery.graphql'
import { appQuery } from './app'

function App() {
  const [ queryReference, loadQuery, disposeQuery ] = useQueryLoader<appMainQuery>(appQuery, null)

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
          {queryReference ? <Movies queryReference={queryReference}/> : null}
        </div>
        <div className="card">
          {queryReference ? <People queryReference={queryReference}/> : null}
        </div>
      </div>
    </>
  )
}

export default App
