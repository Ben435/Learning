import { graphql } from 'react-relay'

export const appQuery = graphql`
  query appMainQuery {
    allFilms @required(action: LOG) {
      edges @required(action: LOG) {
        node @required(action: LOG) {
          id
          ...MovieDetailsFragment
        }
      }
    }
    allPeople @required(action: LOG) {
      edges @required(action: LOG) {
        node @required(action: LOG) {
          id
          ...PersonDetailsFragment
        }
      }
    }
  }
`
