// assign reports or other cases to the specified case by ULID
// this should take an array of {type: report | case, id: ULID}
// recursion of cases should be allowed
// reports are allowed to be assigned to multiple cases
// if a report comes from a queue that is marked as sensitive, this case should also be marked sensitive.
