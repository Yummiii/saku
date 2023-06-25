using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;
using Saku.Models;

namespace Saku.Database.Configurations;

public class UserModelConfiguration : IEntityTypeConfiguration<UserModel>
{
    public void Configure(EntityTypeBuilder<UserModel> builder)
    {
        builder.HasKey(x => x.Id);

        builder.Property(x => x.DiscordId).IsRequired();
        builder.HasIndex(x => x.DiscordId).IsUnique();

        builder.Property(x => x.UserName).HasMaxLength(64).IsRequired();

        builder.Property(x => x.State).HasConversion<ulong>().IsRequired();
    }
}