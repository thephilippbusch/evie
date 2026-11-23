import { type CallOptions, Code, ConnectError } from "@connectrpc/connect";
import { Data, Effect } from "effect";

/** How long one backend call may take. */
const TIMEOUT_MS = 5000;

/** The backend can't be reached or didn't answer in time. */
export class BackendUnavailable extends Data.TaggedError("BackendUnavailable")<{
  readonly cause: ConnectError;
}> {}

/** The backend answered with a status code that the caller didn't turn into an error of its own. */
export class BackendError extends Data.TaggedError("BackendError")<{
  readonly code: Code;
  readonly cause: ConnectError;
}> {}

const toFailure = (cause: unknown): BackendUnavailable | BackendError => {
  const error = ConnectError.from(cause);
  if (error.code === Code.Unavailable || error.code === Code.DeadlineExceeded) {
    return new BackendUnavailable({ cause: error });
  }
  return new BackendError({ code: error.code, cause: error });
};

/** Runs one RPC with a deadline. Interrupting the effect cancels the call. */
export const callBackend = <A>(call: (options: CallOptions) => Promise<A>) =>
  Effect.tryPromise({
    try: (signal) => call({ signal, timeoutMs: TIMEOUT_MS }),
    catch: toFailure,
  });
