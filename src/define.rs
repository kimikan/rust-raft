

//1. lead election:
//    if not received heartbeat message(with term) from Lead.  self become candidate
//   then start to lead election,   term++,  then vote self,  self vote message to others.
//    every node only be allowed to vote once at one term.
//   if recieved n (n>half nodes), then change self to lead,  then send heartbeat messages to others, 
//	 others change self to follower,

//   if not recieved , then term++, random a interval to do next lead election

//2. log replication:
// after received a log added request,  then Lead dispatcher these to others node.
// if half received, then commit.  then ack the client.

//error scenario
//  after combine two partitions into one, 
// if one lead received higher term then self, it becomes follower. downgrade

//error handling

pub enum Role {
    None,
    
    /*only 1 master in a cluster */
    Master,

    /* temp state for lead vote */
    Candidate,
    /* a more general role */
    Follower,
}


pub struct Message {
    pub _id:u32,
}
