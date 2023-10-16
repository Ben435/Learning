import { PreloadedQuery, usePreloadedQuery } from 'react-relay'
import { Movie } from './Movie'
import { appMainQuery } from '../__generated__/appMainQuery.graphql'
import { appQuery } from '../app'

export interface MoviesProps {
    queryReference: PreloadedQuery<appMainQuery>
}

export function Movies({ queryReference }: MoviesProps) {
  const data = usePreloadedQuery(appQuery, queryReference)

  return (
    <>
    {data!.allFilms.edges.map(film => <Movie key={film!.node.id} film={film!.node} />)}
    </>
  )
}