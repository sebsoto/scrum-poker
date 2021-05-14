use std::collections::HashMap;
use std::sync::RwLock;
use std::error::Error;

/// What the end user should be interacting with
pub struct ScrumPoker {
    sessions: RwLock<HashMap<String, Session>>,
    max_sessions: usize,
}

impl ScrumPoker {
    /// Returns a new ScrumPoker instance with the maximum amount of session specified
    pub fn new(max_sessions: usize) -> Self {
        ScrumPoker{ sessions: RwLock::new(HashMap::new()), max_sessions}
    }

    /// Adds a new session to the ScrumPoker instance. Name must be unique to the other sessions.
    pub fn add_session(&self, name: String) -> Result<(), Box<dyn Error>> {
        let mut lock = self.sessions.try_write().unwrap();
        if lock.len() >= self.max_sessions {
            return Err(String::from("too many sessions").into())
        }
        if lock.contains_key(&name) {
            return Err(String::from("session with that name already exists").into())
        }

        lock.insert(name, Session::new(String::from("default")));
        Ok(())
    }

    pub fn list_sessions(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let lock = self.sessions.try_read().unwrap();
        Ok(lock.keys().cloned().collect())

    }

    /// Sends in a vote for a specified session. Only the last vote for each voter will be counted.
    pub fn vote(&self, session_name: &str, voter_name: String, vote: usize) -> Result<(), Box<dyn Error>> {
        let mut lock = self.sessions.try_write().unwrap();
        let session = match lock.get_mut(session_name) {
            Some(session) => session,
            None => return Err(String::from("session does not exist").into())
        };
        session.add_vote(voter_name, vote);
        Ok(())
    }

    /// Returns the voting results of the current topic.
    pub fn get_results(&self, session_name: &str) -> Result<Vec<(String, usize)>, Box<dyn Error>> {
        let lock = self.sessions.try_read().unwrap();
        let session = match lock.get(session_name) {
            Some(session) => session,
            None => return Err(String::from("session does not exist").into())
        };

        let mut votes = Vec::new();
        for vote in session.votes.iter() {
            votes.push((vote.0.clone(), *vote.1));
        }
        Ok(votes)
    }

    /// Clears the votes and sets the topic name
    pub fn new_topic(&self, session_name: &str, topic: String) -> Result<(), Box<dyn Error>> {
        let mut lock = self.sessions.try_write().unwrap();
        let session = match lock.get_mut(session_name) {
            Some(session) => session,
            None => return Err(String::from("session does not exist").into())
        };
        *session = Session::new(topic);
        return Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::server::{ScrumPoker, Session};

    #[test]
    fn test_add_session() {
        let mut sp = ScrumPoker::new(1);
        sp.add_session(String::from("1")).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_max_session_limit() {
        let mut sp = ScrumPoker::new(1);
        assert_eq!(sp.add_session(String::from("1")), Ok(()));
        sp.add_session(String::from("2")).unwrap();
    }

    #[test]
    fn test_vote() {
        let voter1 = "me";
        let vote1: usize = 5;
        let voter2 = "you";
        let vote2: usize = 8;
        let session_name = "session1";
        let mut expected_votes = vec![(voter1.to_string(), vote1), (voter2.to_string(), vote2)];

        let mut sp = ScrumPoker::new(1);
        assert_eq!(sp.add_session(String::from(session_name)), Ok(()));

        // voter1 decides to change their vote, only last vote should be used
        assert_eq!(sp.vote(session_name, String::from(voter1),vote2), Ok(()));
        assert_eq!(sp.vote(session_name, String::from(voter1),vote1), Ok(()));
        assert_eq!(sp.vote(session_name, String::from(voter2),vote2), Ok(()));


        // get results and check that they are equal to what is expected.
        let mut results = sp.get_results(session_name).unwrap();
        expected_votes.sort();
        results.sort();
        assert_eq!(results, expected_votes);
    }
}

struct Session {
    topic: String,
    // using a HashMap because I dont want to learn how to do sessions right now, so I'm going to
    // make the user send their name along with their vote, with the limitation of 1 vote per user.
    votes: HashMap<String, usize>
}

impl Session {
    fn new(topic: String) -> Self {
        Session{topic, votes: HashMap::new()}
    }

    fn add_vote(&mut self, voter_name: String, vote: usize) {
        self.votes.insert(voter_name, vote);
    }
}
