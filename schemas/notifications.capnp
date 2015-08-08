@0xd5eaea9663911446;
using Common = import "common.capnp";

struct Notification {
  union {
    newEntity :group {
      id @0: UInt64;
    }

    entityLeavesMap :group {
      id @1: UInt64;
    }

    entityMessage :group {
      id @2: UInt64;
      message @3: Text;
    }

    entityWalk :group {
      id @4: UInt64;
      orientation @5: Common.Direction;
    }


# TODO: Remove that part once we have UDP
    gameTick :group {
      instance @6: State;
    }
  }
}

struct State {
  tickId @0: UInt64;
  entities @1 :List(Entity);
}

struct Location {
  x @0: UInt32;
  y @1: UInt32;
}

struct Entity {
  id @0: UInt64;
  location @1: Location;
  orientation @2: Common.Direction;
}
  
