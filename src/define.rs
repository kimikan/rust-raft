

enum Role {
    /*only 1 master in a cluster */
    Master,

    /* temp state for lead vote */
    Candidate,
    /* a more general role */
    Follower,
    Empty,
}

