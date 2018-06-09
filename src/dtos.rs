use dmos;
use serde_formats;
use serde_derive;

type ItemCount = u32;

// We use some keywords in the definition of DTOS:
// Partial: There is no Primary Id specified. And foreign keys are not resolved. Usually used in Post and Put
// Unresolved: Only Ids without content.
// ############### Response/Outer DTOS #########################

#[derive(Serialize)]
#[serde(rename_all="lowercase")]
pub struct GetRpgSystems{
    pub rpgsystems: Vec<dmos::RpgSystem>,
}

#[derive(Serialize)]
#[serde(rename_all="lowercase")]
pub struct GetRpgSystem{
    pub rpgsystems: Vec<RpgSystemWithTitles>,
}

#[derive(Deserialize)]
#[serde(rename_all="lowercase")]
pub struct PutPostRpgSystem{
    pub rpgsystem: PartialRpgSystem
}

#[derive(Serialize)]
#[serde(rename_all="lowercase")]
pub struct GetTitles{
    pub titles: Vec<TitleWithSystem>,
}

#[derive(Serialize)]
#[serde(rename_all="lowercase")]
pub struct GetTitle{
    pub title: TitleWithSystemWithBooks,
}

#[derive(Deserialize)]
#[serde(rename_all="lowercase")]
pub struct PutPostTitle{
    pub title: PartialTitle
}

#[derive(Serialize)]
#[serde(rename_all="lowercase")]
pub struct GetBooks{
    pub books: Vec<BookWithTitleWithOwnerWithRental>,
}

#[derive(Serialize)]
#[serde(rename_all="lowercase")]
pub struct GetBook{
    pub book: BookWithTitleWithOwnerWithRental,
}

#[derive(Deserialize)]
#[serde(rename_all="lowercase")]
pub struct PutPostBook{
    pub book: PartialBook,
}

#[derive(Serialize)]
#[serde(rename_all="lowercase")]
pub struct GetMembers{
    pub members: Vec<MemberWithRoles>,
}

#[derive(Serialize)]
#[serde(rename_all="lowercase")]
pub struct GetMember{
    pub member: MemberWithRoles,
}

#[derive(Serialize)]
#[serde(rename_all="lowercase")]
pub struct GetMemberInventory{
    pub member: Entity,
    pub inventory: Inventory,
}

#[derive(Deserialize)]
#[serde(rename_all="lowercase")]
pub struct PutMemberInvetory{
    pub book: PartialBook,
}

#[derive(Serialize)]
#[serde(rename_all="lowercase")]
pub struct GetGuilds{
    pub guilds: Vec<GuildWithContact>,
}

#[derive(Serialize)]
#[serde(rename_all="lowercase")]
pub struct GetGuild{
    pub guild: GuildWithContact,
}

#[derive(Serialize)]
#[serde(rename_all="lowercase")]
pub struct GetGuildInventory{
    pub guild: Entity,
    pub inventory: Inventory,
}

#[derive(Deserialize)]
#[serde(rename_all="lowercase")]
pub struct PutPostGuild{
    pub guild: PartialGuild,
}

// ############### Inner DTOS ############################
#[derive(Serialize)]
pub struct RpgSystemWithTitles{
    pub id: dmos::RpgSystemId,
    pub name: String,
    pub titles: Vec<dmos::Title>
}

#[derive(Deserialize)]
pub struct PartialRpgSystem {
    pub name: String,
}

#[derive(Serialize)]
pub struct TitleWithSystem{
    pub id: dmos::TitleId,
    pub name: String,
    pub system: dmos::RpgSystem,
    pub language: String,
    pub publisher: String,
    pub year: dmos::Year,
    pub coverimage: Option<String>,
    pub stock: ItemCount,
    pub available: ItemCount,
}

#[derive(Serialize)]
pub struct TitleWithSystemWithBooks{
    pub id: dmos::TitleId,
    pub name: String,
    pub system: dmos::RpgSystem,
    pub language: String,
    pub publisher: String,
    pub year: dmos::Year,
    pub coverimage: Option<String>,
    pub stock: ItemCount,
    pub available: ItemCount,
    pub books: Vec<BookWithOwnerWithRental>
}

#[derive(Serialize)]
pub struct BookWithOwnerWithRental {
    pub id: dmos::BookId,
    pub owner: Entity,
    pub quality: String,
    pub available: bool,
    pub rental: Rental,
}

#[derive(Serialize)]
pub struct Entity {
    #[serde(rename="type")]
    pub entity_type: dmos::EntityType,
    pub id: dmos::Id,
    pub name: String,
}

#[derive(Deserialize)]
pub struct UnresolvedEntity {
    #[serde(rename="type")]
    pub entity_type: dmos::EntityType,
    pub id: dmos::Id,
}

#[derive(Serialize)]
pub struct Rental {
    #[serde(with = "serde_formats::naive_date")]
    pub from: dmos::Date,
    #[serde(with = "serde_formats::naive_date")]
    pub to: dmos::Date,
    pub rentee: Entity,
}

#[derive(Deserialize)]
pub struct PartialTitle {
    pub name: String,
    pub system: dmos::TitleId,
    pub language: String,
    pub publisher: String,
    pub year: dmos::Year,
    pub coverimage: Option<String>,
}

#[derive(Serialize)]
pub struct BookWithTitleWithOwnerWithRental {
    pub id: dmos::BookId,
    pub title: TitleWithSystem,
    pub owner: Entity,
    pub quality: String,
    pub available: bool,
    pub rental: Rental,
}

#[derive(Deserialize)]
pub struct PartialBook {
    pub title: dmos::TitleId,
    pub owner: UnresolvedEntity,
    pub quality: String,
}

#[derive(Serialize)]
pub struct Inventory {
    pub ownedbookds: Vec<BookWithTitleWithOwnerWithRental>,
    pub rentedbooks: Vec<BookWithOwnerWithRental>,
}

#[derive(Serialize)]
pub struct MemberWithRoles {
    pub id: dmos::MemberId,
    pub name: String,
    pub email: String,
    pub roles: Vec<dmos::Role>
}

#[derive(Serialize)]
pub struct Member {
    pub id: dmos::MemberId,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct GuildWithContact {
    pub id: dmos::GuildId,
    pub name: String,
    pub address: String,
    pub contact: Member
}

#[derive(Deserialize)]
pub struct PartialGuild {
    pub name: String,
    pub address: String,
    pub contact: dmos::MemberId
}
