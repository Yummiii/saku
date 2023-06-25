using Microsoft.EntityFrameworkCore.Metadata;
using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace Saku.Migrations
{
    public partial class AddChannelTable : Migration
    {
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.CreateTable(
                name: "Channels",
                columns: table => new
                {
                    DiscordChannelId = table.Column<ulong>(type: "bigint unsigned", nullable: false)
                        .Annotation("MySql:ValueGenerationStrategy", MySqlValueGenerationStrategy.IdentityColumn),
                    State = table.Column<ulong>(type: "bigint unsigned", nullable: false),
                    System = table.Column<string>(type: "text", nullable: true)
                        .Annotation("MySql:CharSet", "utf8mb4"),
                    Model = table.Column<int>(type: "int", nullable: false),
                    BoundUserId = table.Column<int>(type: "int", nullable: true)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_Channels", x => x.DiscordChannelId);
                    table.ForeignKey(
                        name: "FK_Channels_Users_BoundUserId",
                        column: x => x.BoundUserId,
                        principalTable: "Users",
                        principalColumn: "Id");
                })
                .Annotation("MySql:CharSet", "utf8mb4");

            migrationBuilder.CreateIndex(
                name: "IX_ChatContexts_ChannelId",
                table: "ChatContexts",
                column: "ChannelId");

            migrationBuilder.CreateIndex(
                name: "IX_Channels_BoundUserId",
                table: "Channels",
                column: "BoundUserId");

            migrationBuilder.AddForeignKey(
                name: "FK_ChatContexts_Channels_ChannelId",
                table: "ChatContexts",
                column: "ChannelId",
                principalTable: "Channels",
                principalColumn: "DiscordChannelId",
                onDelete: ReferentialAction.Cascade);
        }

        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropForeignKey(
                name: "FK_ChatContexts_Channels_ChannelId",
                table: "ChatContexts");

            migrationBuilder.DropTable(
                name: "Channels");

            migrationBuilder.DropIndex(
                name: "IX_ChatContexts_ChannelId",
                table: "ChatContexts");
        }
    }
}
