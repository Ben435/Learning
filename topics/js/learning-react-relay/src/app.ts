import { graphql } from "react-relay";

export const appQuery = graphql`
  query appMainQuery {
    allFilms {
      films {
        id
        ...MovieDetailsFragment
      }
    }
    allPeople {
      people {
          id
          ...PersonDetailsFragment
      }
    }
  }
`
