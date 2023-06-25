using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Metadata.Builders;
using Saku.Models;

namespace Saku.Database.Configurations;

public class ChatContextModelConfiguration : IEntityTypeConfiguration<ChatContextModel>
{
    public void Configure(EntityTypeBuilder<ChatContextModel> builder)
    {
        builder.HasKey(x => x.Id);

        builder.Property(x => x.ChatType).HasConversion<int>().IsRequired();

        builder.Property(x => x.Message).HasColumnType("longtext").IsRequired();

        builder.Property(x => x.IsPresentInCurrentContext).IsRequired();
        builder.HasIndex(x => x.IsPresentInCurrentContext);

        builder.Property(x => x.CreatedAt).IsRequired();

        builder.HasOne(x => x.User).WithMany(x => x.ChatContexts).HasForeignKey(x => x.UserId);
        builder.HasOne(x => x.Channel).WithMany(x => x.ChatContextModels).HasForeignKey(x => x.ChannelId).IsRequired();
    }
}