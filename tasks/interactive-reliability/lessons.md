# Lessons

- The recorded session is decisive evidence: both accepted assistant turns ended
  in short alphabetic fragments, while a separate empty turn triggered recovery.
- The root cause was protocol-level: both SSE decoders and the session loop
  treated bare transport EOF as normal completion.
