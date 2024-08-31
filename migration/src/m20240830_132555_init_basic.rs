use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Organization::Table)
                    .if_not_exists()
                    .col(pk_auto(Organization::OrganizationID))
                    .col(string(Organization::Name))
                    .col(string(Organization::Location))
                    .col(string(Organization::License))
                    .col(
                        ColumnDef::new(Organization::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Organization::UpdatedAt).timestamp())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::UserID))
                    .col(ColumnDef::new(User::OrganizationID).integer())
                    .col(string(User::FirstName))
                    .col(string(User::LastName))
                    .col(string(User::Email))
                    .col(string(User::PhoneNumber))
                    .col(string(User::City))
                    .col(
                        ColumnDef::new(User::UserType)
                            .string()
                            .not_null()
                            .default("user"),
                    )
                    .col(string(User::Password))
                    .col(ColumnDef::new(User::PasswordCode).integer())
                    .col(string(User::ProfilePicture))
                    .col(string(User::Language))
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(User::UpdatedAt).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_organization")
                            .from(User::Table, User::OrganizationID)
                            .to(Organization::Table, Organization::OrganizationID),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Event::Table)
                    .if_not_exists()
                    .col(pk_auto(Event::EventID))
                    .col(ColumnDef::new(Event::OrganizationID).integer())
                    .col(ColumnDef::new(Event::Name).string().default("").not_null())
                    .col(
                        ColumnDef::new(Event::Location)
                            .string()
                            .default("")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Event::EventType)
                            .string()
                            .default("")
                            .not_null(),
                    )
                    .col(ColumnDef::new(Event::Date).string().default("").not_null())
                    .col(ColumnDef::new(Event::Image).string().default("").not_null())
                    .col(
                        ColumnDef::new(Event::IsPublic)
                            .boolean()
                            .default(1)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Event::Status)
                            .string()
                            .default("")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Event::TicketSaleUrl)
                            .string()
                            .default("")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Event::ActiveFrom)
                            .string()
                            .default("")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Event::ActiveTo)
                            .string()
                            .default("")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Event::TrendingScore)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Event::TicketMaxPrice).integer())
                    .col(
                        ColumnDef::new(Event::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Event::UpdatedAt).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_event_organization")
                            .from(Event::Table, Event::OrganizationID)
                            .to(Organization::Table, Organization::OrganizationID),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Ticket::Table)
                    .if_not_exists()
                    .col(pk_auto(Ticket::TicketID))
                    .col(integer(Ticket::UserID))
                    .col(integer(Ticket::EventID))
                    .col(string(Ticket::Header))
                    .col(string(Ticket::Description))
                    .col(integer(Ticket::Price))
                    .col(integer(Ticket::Quantity))
                    .col(boolean(Ticket::RequiresMembership))
                    .col(string(Ticket::Association))
                    .col(boolean(Ticket::IsSelling))
                    .col(
                        ColumnDef::new(Ticket::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Ticket::UpdatedAt).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ticket_user")
                            .from(Ticket::Table, Ticket::UserID)
                            .to(User::Table, User::UserID),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ticket_event")
                            .from(Ticket::Table, Ticket::EventID)
                            .to(Event::Table, Event::EventID),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Advertisement::Table)
                    .if_not_exists()
                    .col(pk_auto(Advertisement::AdvertisementID))
                    .col(string(Advertisement::Advertiser))
                    .col(string(Advertisement::ContentHtml))
                    .col(boolean(Advertisement::IsActive))
                    .col(integer(Advertisement::Views))
                    .col(integer(Advertisement::Clicks))
                    .col(string(Advertisement::RedirectUrl))
                    .col(string(Advertisement::Type))
                    .col(string(Advertisement::Location))
                    .col(
                        ColumnDef::new(Advertisement::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Advertisement::UpdatedAt).timestamp())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Organization::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Ticket::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Advertisement::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Organization {
    Table,
    OrganizationID,
    Name,
    Location,
    License,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum User {
    Table,
    UserID,
    OrganizationID,
    FirstName,
    LastName,
    Email,
    PhoneNumber,
    City,
    UserType,
    Password,
    PasswordCode,
    ProfilePicture,
    Language,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Event {
    Table,
    EventID,
    OrganizationID,
    Name,
    Location,
    EventType,
    Date,
    Image,
    IsPublic,
    Status,
    TicketSaleUrl,
    ActiveFrom,
    ActiveTo,
    TrendingScore,
    TicketMaxPrice,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Ticket {
    Table,
    TicketID,
    UserID,
    EventID,
    Header,
    Description,
    Price,
    Quantity,
    RequiresMembership,
    Association,
    IsSelling,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Advertisement {
    Table,
    AdvertisementID,
    Advertiser,
    ContentHtml,
    IsActive,
    Views,
    Clicks,
    RedirectUrl,
    Type,
    Location,
    CreatedAt,
    UpdatedAt,
}
