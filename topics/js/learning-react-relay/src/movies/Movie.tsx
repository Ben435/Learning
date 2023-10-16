import { graphql, useFragment } from "react-relay"
import { MovieDetailsFragment$key } from "./__generated__/MovieDetailsFragment.graphql"

const getFilmDetails = graphql`
    fragment MovieDetailsFragment on Film {
        title
        director
    }
`

export interface MovieProps {
  film: MovieDetailsFragment$key
}

export function Movie({ film }: MovieProps) {
  const filmDetails = useFragment(getFilmDetails, film)

  return <p>Movie {filmDetails.title} directed by {filmDetails.director}</p>
}