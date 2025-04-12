set -e

echo "Starting to test Kubernetes Deployment YAML files..."
for file in manifests/*.yaml; do
  echo "Testing file: $file"
  # Apply the YAML in dry-run mode to check for errors
  kubectl apply -f $file --dry-run=server || echo "Error: Testing failed for $file"
  echo ""
done
echo "All tests completed