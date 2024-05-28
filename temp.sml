datatype 'a Maybe = None | Some of 'a

fun twoMaybes (a : int Maybe) (b : int Maybe) =
  case a of
    Some va =>
      case b of
        None => 0
      | Some vb => va + vb
  | None => 0
