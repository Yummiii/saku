using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace Saku.Migrations
{
    public partial class AddIndexinChatContextIsPresentInCurrentContext : Migration
    {
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.CreateIndex(
                name: "IX_ChatContexts_IsPresentInCurrentContext",
                table: "ChatContexts",
                column: "IsPresentInCurrentContext");
        }

        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropIndex(
                name: "IX_ChatContexts_IsPresentInCurrentContext",
                table: "ChatContexts");
        }
    }
}
