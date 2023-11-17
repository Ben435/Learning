import { PreloadedQuery, usePreloadedQuery } from 'react-relay'
import { Movie } from './Movie'
import { appMainQuery } from '../__generated__/appMainQuery.graphql'
import { appQuery } from '../app'
import { Planet } from './planet/Planet'

export interface MoviesProps {
    queryReference: PreloadedQuery<appMainQuery>
}

export function Movies({ queryReference }: MoviesProps) {
  const data = usePreloadedQuery(appQuery, queryReference)

  return (
    <>
    {data!.allFilms.edges.map(film => (
    <div>
      <Movie key={film!.node.id} film={film!.node} />
      {film!.node!.planetConnection!.edges!.map((edge) => <Planet key={`${film!.node.id}-${edge!.node!.id}`} planet={edge!.node!} />)}
    </div>
    ))}
    </>
  )
}