import { PreloadedQuery, usePreloadedQuery } from 'react-relay'
import { Suspense } from 'react'
import { Movie } from './Movie'
import { AppQuery } from '../__generated__/AppQuery.graphql'
import { appQuery } from '../appQuery'

export interface MoviesProps {
    queryReference: PreloadedQuery<AppQuery>
}

export function Movies({ queryReference }: MoviesProps) {
  const data = usePreloadedQuery(appQuery, queryReference)

  return (
    <Suspense fallback={<p>loading movies...</p>}>
        {data.allFilms!.films!.map(film => <Movie key={film!.id} film={film!} />)}
    </Suspense>
  )
}