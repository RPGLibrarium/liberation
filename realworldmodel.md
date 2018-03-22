# Objects
## Book
- Title: String
- Owner: Entity
- Rentals: List\<Rental\>
- Quality: Enum

# Title:
- Name: String
- System: RpgSystem
- Language: String
- Publisher: String
- Year: String
- Coverimage: String

# Rental:
- From: Date
- To: Date
- Rentee: Owner

## RpgSystem
- Name: String

## Entity
### Members
- Username: String
- Name: String
- EMail: String
- Role: Role

### Guild
- Name: String
- Adress: String
- Contactperson: Member
