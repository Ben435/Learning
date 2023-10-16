import { graphql } from "react-relay";

export const appQuery = graphql`
  query AppQuery {
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
