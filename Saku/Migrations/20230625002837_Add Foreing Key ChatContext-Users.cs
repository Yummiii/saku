using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace Saku.Migrations
{
    public partial class AddForeingKeyChatContextUsers : Migration
    {
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.AlterColumn<int>(
                name: "UserId",
                table: "ChatContexts",
                type: "int",
                nullable: true,
                oldClrType: typeof(ulong),
                oldType: "bigint unsigned",
                oldNullable: true);

            migrationBuilder.CreateIndex(
                name: "IX_ChatContexts_UserId",
                table: "ChatContexts",
                column: "UserId");

            migrationBuilder.AddForeignKey(
                name: "FK_ChatContexts_UserModel_UserId",
                table: "ChatContexts",
                column: "UserId",
                principalTable: "UserModel",
                principalColumn: "Id");
        }

        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropForeignKey(
                name: "FK_ChatContexts_UserModel_UserId",
                table: "ChatContexts");

            migrationBuilder.DropIndex(
                name: "IX_ChatContexts_UserId",
                table: "ChatContexts");

            migrationBuilder.AlterColumn<ulong>(
                name: "UserId",
                table: "ChatContexts",
                type: "bigint unsigned",
                nullable: true,
                oldClrType: typeof(int),
                oldType: "int",
                oldNullable: true);
        }
    }
}
