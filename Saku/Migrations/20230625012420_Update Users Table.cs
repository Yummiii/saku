using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace Saku.Migrations
{
    public partial class UpdateUsersTable : Migration
    {
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropForeignKey(
                name: "FK_ChatContexts_UserModel_UserId",
                table: "ChatContexts");

            migrationBuilder.DropPrimaryKey(
                name: "PK_UserModel",
                table: "UserModel");

            migrationBuilder.RenameTable(
                name: "UserModel",
                newName: "Users");

            migrationBuilder.RenameIndex(
                name: "IX_UserModel_DiscordId",
                table: "Users",
                newName: "IX_Users_DiscordId");

            migrationBuilder.AddPrimaryKey(
                name: "PK_Users",
                table: "Users",
                column: "Id");

            migrationBuilder.AddForeignKey(
                name: "FK_ChatContexts_Users_UserId",
                table: "ChatContexts",
                column: "UserId",
                principalTable: "Users",
                principalColumn: "Id");
        }

        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropForeignKey(
                name: "FK_ChatContexts_Users_UserId",
                table: "ChatContexts");

            migrationBuilder.DropPrimaryKey(
                name: "PK_Users",
                table: "Users");

            migrationBuilder.RenameTable(
                name: "Users",
                newName: "UserModel");

            migrationBuilder.RenameIndex(
                name: "IX_Users_DiscordId",
                table: "UserModel",
                newName: "IX_UserModel_DiscordId");

            migrationBuilder.AddPrimaryKey(
                name: "PK_UserModel",
                table: "UserModel",
                column: "Id");

            migrationBuilder.AddForeignKey(
                name: "FK_ChatContexts_UserModel_UserId",
                table: "ChatContexts",
                column: "UserId",
                principalTable: "UserModel",
                principalColumn: "Id");
        }
    }
}
