fn main() {
    println!("Hello, world!");
}

struct Logger {
}

impl Logger {
    fn log(&self, message: &str) {
        println!("{}", message)
    }
}

struct Storage {
}

impl Storage {
    fn GetMatchHandle(&self, match_id: &str) -> &MatchHandle {}

    fn GetMatchPlayerBoard(&self, match_id: &str, player_id: &str) -> [CardID] {}
    fn GetMatchPlayerHand(&self, match_id: &str, player_id: &str) -> [CardID] {}
    fn GetMatchPlayerDeck(&self, match_id: &str, player_id: &str) -> [CardID] {}
    fn UpdateMatchPlayerHand(&self, match_id: &str, player_id: &str, new_hand: [CardID]) {}
    fn UpdateMatchPlayerBoard(&self, match_id: &str, player_id: &str, new_hand: [CardID]) {}
    fn UpdateMatchPlayerDeck(&self, match_id: &str, player_id: &str, new_hand: [CardID]) {}
}

struct GameState {
}

impl GameState {
    fn setup(&self) {
    }
}

struct MatchHandle {
    logger: Logger,
    state: GameState,
    storage: Storage,
    id: String,
}

impl MatchHandle {
    fn ProcessActions(&self, user_id: &str, actions: &[UserAction]){}
    fn ProcessAction(&self, user_id: &str, action: UserAction){
        match action {
            UserAction::Ready => {
                // reveal and starts resolving
            }
            UserAction::Surrender => {
                // game over
            }
            UserAction::PlaceCard(hand_index) => {
                // update board and hand
                // a.board.append (some card)

                // personalized events (face down)
            }
        }
    }
    fn UnloadEvents(&self, user_id: &str, last_event_received: i64) -> &[ServerEvents] {}
}

struct ActionHandler {
    storage: Storage,
}

impl ActionHandler {
    fn HandleSync(
        &self, user_id: &str,
        match_id: &str, 
        actions: &[UserAction],
        last_event_received: i64,
    ) -> &[ServerEvents] {
        let match_handle: &MatchHandle = self.storage.GetMatchHandle(match_id);
        // TODO: check user belongs to this match
        match_handle.ProcessActions(user_id, actions); // this can be async, but sync is fine
        let events: &[ServerEvents] = match_handle.UnloadEvents(user_id, last_event_received);
        events
    }
}

enum UserAction {
    PlaceCard(i32),
    Ready,
    Surrender,
}

enum ServerEvents {
    GameStarted,
    RoundStarted,
    PhaseStarted,
    StaminaChanged{ player_id: String, amount: i32, capcacity: i32 },
    HealthChanged{ player_id: String, amount: i32, capacity: i32 },
    SelfCardDrawn(CardID), 
    OpponentCardDrawn(u32), // assuming 2 players per game
    OpponentCardPlayed(u32), // just a count, because cards are played face down
    OpponentPlayedCardRevealed{ index: i32, card: CardID },
    GameOver{ winner_id: String },
}

enum CardID {
    LightPunch,
    HeavyPunch,
    Dodge,
    Block,
}