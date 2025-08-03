

# Value fields in lab and test do have empty strings.
  1. **Empty vs Missing Values**: When a lab/test "Value" field is empty string, does this mean "no result recorded" or "result is literally empty/zero"?

  2. **Database Storage**: How are empty values stored in the database - as empty strings or NULLs?

  3. **Downstream Impact**: Do existing Python consumers expect empty strings or null values in JSON responses?

  4. **Business Rules**: Are there fields that should never be empty vs. fields where empty is valid?

  5. **Data Cleaning**: Should we trim whitespace from incoming data, or preserve it exactly as received?

  6. **Validation Strictness**: Should validation reject records with empty required fields, or just log warnings?
