import { useQueryLoader } from 'react-relay'
import './App.css'
import { Movies } from './movies/Movies'
import { People } from './people/People'
import { useEffect } from 'react'
import { AppQuery } from './__generated__/AppQuery.graphql'
import { appQuery } from './appQuery'

function App() {
  const [ queryReference, loadQuery, disposeQuery ] = useQueryLoader<AppQuery>(appQuery, null)

  useEffect(() => {
    if (queryReference == null) {
      loadQuery({})
    }
  }, [queryReference, loadQuery, disposeQuery])
  

  return (
    <>
      <h1>Star Wars stuff</h1>
      <div className="card">
        {queryReference ? <Movies queryReference={queryReference}/> : null}
        {queryReference ? <People queryReference={queryReference}/> : null}
      </div>
    </>
  )
}

export default App
